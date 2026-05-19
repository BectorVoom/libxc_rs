//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1146/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1146<F: Float>(t11195: F, t11204: F, t11211: F, t11213: F, t14702: F, t14708: F, t14713: F, t14759: F, t14779: F, t14784: F, t14787: F, t14790: F, t14793: F, t14796: F, t14799: F, t14802: F, t14805: F, t14868: F, t14870: F, t14887: F, t14890: F, t14911: F) -> F {
    let t14913 = -t11195 - t11204 + F::cast_from(0.13287407407407407408e0_f64) * t14702 - t14868 + F::cast_from(0.29896666666666666667e0_f64) * t14708 - t14870 + F::cast_from(0.82156666666666666667e-1_f64) * t14713 + F::new(0.1898925e1) * t14759 + F::cast_from(0.18257037037037037037e0_f64) * t11211 + F::cast_from(0.18257037037037037037e-1_f64) * t11213 + t14887 + F::cast_from(0.36514074074074074075e-1_f64) * t14779 - t14890 - F::cast_from(0.54771111111111111112e-1_f64) * t14784 - F::cast_from(0.27385555555555555556e-1_f64) * t14787 - F::cast_from(0.16431333333333333333e0_f64) * t14790 + F::cast_from(0.32862666666666666666e0_f64) * t14793 + F::cast_from(0.16431333333333333333e0_f64) * t14796 + F::cast_from(0.49293999999999999999e0_f64) * t14799 + F::cast_from(0.142419375e1_f64) * t14802 - F::new(0.76790625e-1) * t14805 + t14911;
    t14913
}
