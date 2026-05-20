//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1911;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1912;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1913;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1914;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta490<F: Float>(t1557: F, t5726: F, t2792: F, t1556: F, t17520: F, t2842: F, t1569: F, t5758: F, t10636: F, t13598: F, t17149: F, t17165: F, t17175: F, t21124: F, t21128: F, t21147: F, t21150: F, t21153: F, t21156: F, t291: F, t10608: F, t324: F, t10832: F, t14276: F, t21259: F, t21263: F, t21265: F, t21267: F, t21270: F, t21302: F, t21305: F, t21306: F, t21309: F, t21312: F, t2861: F, t2886: F, t2905: F, t2930: F, t311: F, t5743: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t21315, t21317, t21318, t21320, t21321, t21334) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1911::<F>(t1557, t5726, t2792, t1556, t17520, t2842, t1569, t5758, t10636, t13598, t17149, t17165, t17175, t21124, t21128, t21147, t21150, t21153, t21156);
        let (t21336, t21347) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1912::<F>(t21334, t291, t10608, t13598, t17149, t17165, t17175, t21124, t21128, t21147, t21150, t21153, t21156);
        let (t21348, t21360) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1913::<F>(t21347, t324, t10832, t13598, t17149, t17165, t17175, t21124, t21128, t21147, t21150, t21153, t21156);
        let t21363 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1914::<F>(t14276, t21259, t21263, t21265, t21267, t21270, t21302, t21305, t21306, t21309, t21312, t21317, t21320, t21321, t21336, t21348, t21360, t2861, t2886, t2905, t2930, t311, t5743);
    (t21315, t21317, t21318, t21320, t21321, t21334, t21336, t21347, t21348, t21360, t21363)
}
