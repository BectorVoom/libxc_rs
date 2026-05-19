//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1185/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1185<F: Float>(t11204: F, t11211: F, t14702: F, t14868: F, t14870: F, t18742: F, t18747: F, t18749: F, t18752: F, t18755: F, t18757: F, t11137: F, t14818: F, t18227: F, t18239: F, t18497: F, t18500: F, t18503: F, t18508: F, t18510: F, t18515: F, t18518: F) -> (F, F) {
    let t18810 = F::new(0.3071625e0) * t18742 - t11204 + F::cast_from(0.26574814814814814815e0_f64) * t14702 - t14868 - t14870 + F::cast_from(0.91285185185185185187e-1_f64) * t11211 - F::new(0.76790625e-1) * t18747 + F::new(0.3071625e0) * t18749 + F::new(0.15358125e0) * t18752 + F::cast_from(0.142419375e1_f64) * t18755 - F::new(0.1898925e1) * t18757;
    let t18832 = F::cast_from(0.11958666666666666667e1_f64) * t18227 + F::cast_from(0.36514074074074074073e-1_f64) * t14818 - F::cast_from(0.27385555555555555556e-1_f64) * t18515 + F::cast_from(0.36514074074074074075e-1_f64) * t18497 + F::cast_from(0.16431333333333333333e0_f64) * t18518 + F::cast_from(0.13287407407407407408e0_f64) * t11137 + F::cast_from(0.59793333333333333334e0_f64) * t18239 - F::cast_from(0.54771111111111111112e-1_f64) * t18503 - F::cast_from(0.16431333333333333333e0_f64) * t18500 + F::cast_from(0.32862666666666666666e0_f64) * t18510 + F::cast_from(0.49293999999999999999e0_f64) * t18508;
    (t18810, t18832)
}
