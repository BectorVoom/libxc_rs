//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta348 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1748;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1749;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta348<F: Float>(t13003: F, t13028: F, t252: F, t1492: F, t2710: F, t1519: F, t2591: F, t225: F, t4266: F, t10049: F, t1528: F, t259: F, t2597: F, t2713: F, t2720: F, t2743: F, t4147: F, t4268: F, t4273: F, t4301: F, t866: F, t9590: F, t9593: F, t1527: F, t2719: F, t10110: F, t4143: F, t2742: F, t2718: F, t4265: F, t798: F, t4145: F, t4142: F, t852: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13029, t13030, t13034, t13036, t13042, t13048) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1748::<F>(t13003, t13028, t252, t1492, t2710, t1519, t2591, t225, t4266, t10049, t1528, t259, t2597, t2713, t2720, t2743, t4147, t4268, t4273, t4301, t866, t9590, t9593);
        let (t13050, t13053, t13059, t13062, t13065, t13068) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1749::<F>(t1527, t2719, t10110, t225, t4143, t2742, t2718, t4265, t798, t4145, t4142, t852);
    (t13029, t13030, t13034, t13036, t13042, t13048, t13050, t13053, t13059, t13062, t13065, t13068)
}
