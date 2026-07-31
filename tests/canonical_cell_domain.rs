use moria::canonical::{
    BRICK_CELL_COUNT, BRICK_EDGE_CELLS, Brick, BrickAabb, BrickCoord, BrickDecodeError,
    CellValidationError, CellWire, DENSE_BRICK_BYTES, LocalCellAabb, LocalCellPoint,
    OccupancyClass, VolumeDomainError,
};
use std::collections::BTreeMap;

#[test]
fn cells_preserve_the_four_byte_little_endian_wire_and_reject_invalid_matter() {
    assert_eq!(core::mem::size_of::<CellWire>(), CellWire::BYTE_LEN);
    let cell = CellWire {
        material_id: 0x1234,
        density_q8_8: -0x2345,
    };
    assert_eq!(cell.to_le_bytes(), [0x34, 0x12, 0xbb, 0xdc]);
    assert_eq!(CellWire::from_le_bytes(cell.to_le_bytes()), cell);

    assert_eq!(
        CellWire {
            material_id: 0,
            density_q8_8: 1,
        }
        .validate_registered(|_| true),
        Err(CellValidationError::EmptyMaterialPositiveDensity)
    );
    assert_eq!(
        CellWire {
            material_id: 7,
            density_q8_8: 0,
        }
        .validate_registered(|_| false),
        Err(CellValidationError::UnregisteredMaterial { material_id: 7 })
    );
    assert!(
        CellWire {
            material_id: 0,
            density_q8_8: 0,
        }
        .validate_registered(|_| false)
        .is_ok()
    );
}

#[test]
fn cells_determine_occupancy_from_registered_immutable_material_facts() {
    let registry = BTreeMap::from([
        (7, OccupancyClass::Never),
        (8, OccupancyClass::SolidAbove { density_q8_8: 10 }),
    ]);

    assert_eq!(
        CellWire {
            material_id: 0,
            density_q8_8: 0,
        }
        .is_occupied(|_| unreachable!()),
        Ok(false)
    );
    assert_eq!(
        CellWire {
            material_id: 7,
            density_q8_8: 1,
        }
        .is_occupied(|id| registry.get(&id).copied()),
        Ok(false)
    );

    for (density_q8_8, expected) in [(9, false), (10, false), (11, true)] {
        assert_eq!(
            CellWire {
                material_id: 8,
                density_q8_8,
            }
            .is_occupied(|id| registry.get(&id).copied()),
            Ok(expected)
        );
    }

    let registry_before_rejection = registry.clone();
    assert_eq!(
        CellWire {
            material_id: 9,
            density_q8_8: 1,
        }
        .is_occupied(|id| registry.get(&id).copied()),
        Err(CellValidationError::UnregisteredMaterial { material_id: 9 })
    );
    assert_eq!(registry, registry_before_rejection);
}

#[test]
fn dense_and_uniform_bricks_have_exact_layout_and_size() {
    assert_eq!(BRICK_EDGE_CELLS, 8);
    assert_eq!(BRICK_CELL_COUNT, 512);
    assert_eq!(DENSE_BRICK_BYTES, 2_048);
    assert!(core::mem::size_of::<Brick>() <= 2 * core::mem::size_of::<usize>());

    let cells = core::array::from_fn(|index| CellWire {
        material_id: index as u16,
        density_q8_8: -(index as i16),
    });
    let dense = Brick::dense(cells);
    assert_eq!(dense.to_dense_le_bytes().len(), DENSE_BRICK_BYTES);
    assert_eq!(&dense.to_dense_le_bytes()[0..4], &[0, 0, 0, 0]);
    assert_eq!(&dense.to_dense_le_bytes()[4..8], &[1, 0, 0xff, 0xff]);
    assert_eq!(dense.cell_at([7, 7, 7]), cells[511]);
    assert_eq!(dense.cell_at([2, 3, 4]), cells[2 + 8 * (3 + 8 * 4)]);
    assert_eq!(dense.cell_at([3, 2, 4]), cells[3 + 8 * (2 + 8 * 4)]);
    assert_eq!(dense.cell_at([2, 4, 3]), cells[2 + 8 * (4 + 8 * 3)]);

    let uniform = CellWire {
        material_id: 3,
        density_q8_8: 4,
    };
    assert!(Brick::uniform(uniform).is_uniform());
    assert_eq!(Brick::uniform(uniform).cell_at([2, 4, 6]), uniform);
}

#[test]
fn dense_brick_decoding_requires_the_exact_payload_length() {
    for length in [0, DENSE_BRICK_BYTES - 1, DENSE_BRICK_BYTES + 1] {
        assert_eq!(
            Brick::try_from_dense_le_bytes(&vec![0; length]),
            Err(BrickDecodeError::InvalidByteLength { actual: length })
        );
    }

    let bytes = [0x55; DENSE_BRICK_BYTES];
    let decoded = Brick::try_from_dense_le_bytes(&bytes).unwrap();
    assert_eq!(decoded.to_dense_le_bytes(), bytes);
}

#[test]
fn uniform_and_dense_empty_and_material_bricks_encode_identically() {
    let empty = CellWire {
        material_id: 0,
        density_q8_8: 0,
    };
    let material = CellWire {
        material_id: 17,
        density_q8_8: 0x1234,
    };

    for cell in [empty, material] {
        assert_eq!(
            Brick::uniform(cell).to_dense_le_bytes(),
            Brick::dense([cell; BRICK_CELL_COUNT]).to_dense_le_bytes()
        );
    }
}

#[test]
fn cell_coordinates_use_euclidean_brick_mapping_including_negatives() {
    assert_eq!(
        BrickCoord::from_local_cell(LocalCellPoint([0, 7, 8])),
        BrickCoord([0, 0, 1])
    );
    assert_eq!(
        BrickCoord::from_local_cell(LocalCellPoint([-1, -8, -9])),
        BrickCoord([-1, -1, -2])
    );
    assert_eq!(LocalCellPoint([-1, -8, -9]).brick_local_index(), [7, 0, 7]);
}

#[test]
fn local_and_brick_aabbs_are_half_open_on_every_edge() {
    let min = LocalCellPoint([-4_095, -4_095, -4_095]);
    let max = LocalCellPoint([4_096, 4_096, 4_096]);
    assert!(LocalCellAabb::try_new(min, max, LocalCellPoint([0, 0, 0])).is_ok());
    assert_eq!(
        LocalCellAabb::try_new(
            LocalCellPoint([3, -4, 10]),
            LocalCellPoint([4, -3, 11]),
            LocalCellPoint([0, 0, 0]),
        ),
        Ok(LocalCellAabb {
            min: LocalCellPoint([3, -4, 10]),
            max: LocalCellPoint([4, -3, 11]),
        })
    );

    for axis in 0..3 {
        for bad_axis_max in [0, -1] {
            let mut bad_max = [1, 1, 1];
            bad_max[axis] = bad_axis_max;
            assert_eq!(
                LocalCellAabb::try_new(
                    LocalCellPoint([0, 0, 0]),
                    LocalCellPoint(bad_max),
                    LocalCellPoint([0, 0, 0])
                ),
                Err(VolumeDomainError::EmptyOrInvertedAxis { axis })
            );
        }

        let mut too_large = [1, 1, 1];
        too_large[axis] = 8_192;
        assert_eq!(
            LocalCellAabb::try_new(
                LocalCellPoint([0, 0, 0]),
                LocalCellPoint(too_large),
                LocalCellPoint([0, 0, 0])
            ),
            Err(VolumeDomainError::SideTooLong { axis })
        );
    }

    assert_eq!(
        LocalCellAabb::try_new(
            LocalCellPoint([-4_096, 0, 0]),
            LocalCellPoint([1, 1, 1]),
            LocalCellPoint([0, 0, 0])
        ),
        Err(VolumeDomainError::PivotRadiusExceeded { axis: 0 })
    );
    assert_eq!(
        LocalCellAabb::try_new(
            LocalCellPoint([0, 0, 0]),
            LocalCellPoint([4_097, 1, 1]),
            LocalCellPoint([0, 0, 0])
        ),
        Err(VolumeDomainError::PivotRadiusExceeded { axis: 0 })
    );

    for axis in 0..3 {
        for bad_axis_max in [0, -1] {
            let mut max = [1, 1, 1];
            max[axis] = bad_axis_max;
            assert_eq!(
                BrickAabb::try_new(BrickCoord([0, 0, 0]), BrickCoord(max)),
                Err(VolumeDomainError::EmptyOrInvertedAxis { axis })
            );
        }
    }
    assert!(
        BrickAabb::try_new(
            BrickCoord([i32::MIN, i32::MIN, i32::MIN]),
            BrickCoord([i32::MAX, i32::MAX, i32::MAX]),
        )
        .is_ok()
    );

    assert_eq!(
        LocalCellAabb::try_new(
            LocalCellPoint([i32::MIN, 0, 0]),
            LocalCellPoint([i32::MIN + 1, 1, 1]),
            LocalCellPoint([i32::MAX, 0, 0]),
        ),
        Err(VolumeDomainError::PivotRadiusExceeded { axis: 0 })
    );
    assert_eq!(
        LocalCellAabb::try_new(
            LocalCellPoint([i32::MIN, 0, 0]),
            LocalCellPoint([i32::MAX, 1, 1]),
            LocalCellPoint([0, 0, 0]),
        ),
        Err(VolumeDomainError::SideTooLong { axis: 0 })
    );
}
