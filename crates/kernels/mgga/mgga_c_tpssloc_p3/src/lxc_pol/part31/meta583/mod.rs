//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1823;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1824;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta583<F: Float>(t22704: F, t5336: F, t80798: F, t22724: F, t26436: F, t81066: F, t26423: F, t81159: F, t215: F, t22839: F, t562: F, t80854: F, t81080: F, t26462: F, t6914: F, t22705: F, t26414: F, t81228: F, t26415: F, t26418: F, t7736: F, t81064: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t90898, t90900, t90903, t90912, t90914, t90915) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1823::<F>(t22704, t5336, t80798, t22724, t26436, t81066, t26423, t81159, t215, t22839, t562, t80854);
        let (t90925, t90956, t90961, t90963, t90970, t90980) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1824::<F>(t81080, t26462, t6914, t22705, t26414, t81228, t26415, t81159, t26418, t7736, t80854, t81064);
    (t90898, t90900, t90903, t90912, t90914, t90915, t90925, t90956, t90961, t90963, t90970, t90980)
}
