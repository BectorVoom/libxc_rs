//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1803;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1804;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta405<F: Float>(t13822: F, t4548: F, t973: F, t2970: F, t4522: F, t6733: F, t884: F, t4531: F, t10254: F, t3961: F, t2988: F, t10236: F, t10235: F, t10186: F, t10233: F, t10267: F, t10274: F, t13806: F, t13813: F, t13817: F, t2960: F, t2986: F, t4523: F, t4532: F, t4549: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t13823, t13825, t13828, t13830, t13831, t13832, t13835, t13836, t13839) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1803::<F>(t13822, t4548, t973, t2970, t4522, t6733, t884, t4531, t10254, t3961, t2988, t10236);
        let (t13840, t13845) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1804::<F>(t10235, t13839, t10186, t10233, t10267, t10274, t13806, t13813, t13817, t13825, t13830, t13832, t13836, t2960, t2986, t4523, t4532, t4549, t973);
    (t13823, t13828, t13831, t13832, t13835, t13836, t13839, t13840, t13845)
}
