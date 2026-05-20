//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta305 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1039;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1040;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1041;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1042;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta305<F: Float>(t1055: F, t21662: F, t1603: F, t5914: F, t1634: F, t5919: F, t10165: F, t21480: F, t381: F, t1625: F, t5848: F, t21614: F, t349: F, t5943: F, t3174: F, t1052: F, t1635: F, t17575: F, t17588: F, t18074: F, t388: F, t4557: F, t4660: F, t5920: F, t5944: F, t1070: F, t193: F, t21251: F, t21255: F, t21263: F, t21265: F, t21267: F, t21270: F, t21302: F, t21305: F, t21317: F, t21320: F, t21336: F, t21591: F, t336: F, t25: F, t265: F, t394: F, t21076: F, t21381: F, t1408: F, t1409: F, t1534: F, t1642: F, t20216: F, t20217: F, t396: F, t40: F, t5397: F, t5398: F, t5669: F, t5955: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t21663, t21669, t21677, t21682, t21684, t21689) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1039::<F>(t1055, t21662, t1603, t5914, t1634, t5919, t10165, t21480, t381, t1625, t5848, t21614, t349);
        let (t21692, t21697) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1040::<F>(t1634, t5943, t3174, t1052, t1635, t17575, t17588, t18074, t21663, t21669, t21677, t21682, t21684, t21689, t388, t4557, t4660, t5920, t5944);
        let t21701 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1041::<F>(t1070, t193, t21251, t21255, t21263, t21265, t21267, t21270, t21302, t21305, t21317, t21320, t21336, t21591, t21697, t336);
        let (t21703, t21713) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1042::<F>(t25, t265, t394, t21076, t21381, t21701, t1408, t1409, t1534, t1642, t20216, t20217, t396, t40, t5397, t5398, t5669, t5955, dens_threshold, rho0, zeta_threshold);
    (t21663, t21669, t21677, t21682, t21684, t21689, t21692, t21697, t21703, t21713)
}
