//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta588 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2327;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2328;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2329;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2330;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta588<F: Float>(t12283: F, t6396: F, t19871: F, t3805: F, t3807: F, t16306: F, t6394: F, t16305: F, t16225: F, t16311: F, t1825: F, t5308: F, t16224: F, t12286: F, t1341: F, t16239: F, t16241: F, t16269: F, t16290: F, t16294: F, t16317: F, t16325: F, t16331: F, t16338: F, t16341: F, t19868: F, t19873: F, t19876: F, t3778: F, t3803: F, t5246: F, t5252: F, t6390: F, t6417: F, t1362: F, t19815: F, t3799: F, t6422: F, t1307: F, t6330: F, t12351: F, t820: F, t1799: F, t5187: F, t3870: F, t1367: F, t19631: F, t16336: F, t1831: F, t12308: F, t12325: F, t12330: F, t12335: F, t1363: F, t1369: F, t16321: F, t16346: F, t16350: F, t16354: F, t3783: F, t5240: F, t5310: F, t5314: F, t6427: F, t6431: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t19879, t19882, t19886, t19890, t19893) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2327::<F>(t12283, t6396, t19871, t3805, t3807, t16306, t6394, t16305, t16225, t16311, t1825, t5308);
        let (t19894, t19899) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2328::<F>(t16224, t19893, t12286, t1341, t16239, t16241, t16269, t16290, t16294, t16317, t16325, t16331, t16338, t16341, t19868, t19873, t19876, t19879, t19882, t19886, t19890, t3778, t3803, t5246, t5252, t6390, t6417);
        let (t19904, t19915, t19917, t19921, t19924, t19926) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2329::<F>(t1362, t19815, t3799, t6417, t6422, t1307, t6330, t12351, t820, t1799, t5187, t3870);
        let (t19930, t19939) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2330::<F>(t1367, t19631, t820, t16336, t1831, t12308, t12325, t12330, t12335, t1363, t1369, t16321, t16346, t16350, t16354, t19904, t19915, t19917, t19921, t19926, t3778, t3783, t5240, t5310, t5314, t6422, t6427, t6431);
    (t19882, t19886, t19890, t19894, t19899, t19904, t19921, t19924, t19926, t19930, t19939)
}
