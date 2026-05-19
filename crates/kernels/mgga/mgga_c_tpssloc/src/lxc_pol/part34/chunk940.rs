//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 940/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk940<F: Float>(t136: F, t21801: F, t11243: F, t21785: F, t21760: F, t21764: F, t21767: F, t21771: F, t21774: F, t21778: F, t21781: F, t21783: F, t21786: F, t21789: F, t21792: F, t21795: F) -> (F, F, F) {
    let t21802 = t136 * t21801;
    let t21804 = t11243 * t21785;
    let t21808 = F::new(0.3071625e0) * t21781 + F::new(0.1898925e1) * t21783 + F::cast_from(0.142419375e1_f64) * t21786 - F::cast_from(0.16431333333333333333e0_f64) * t21789 + F::cast_from(0.49293999999999999999e0_f64) * t21792 + F::cast_from(0.82156666666666666667e-1_f64) * t21795 + F::cast_from(0.33218518518518518518e0_f64) * t21760 - F::cast_from(0.11958666666666666667e1_f64) * t21764 + F::new(0.17938e1) * t21771 + F::cast_from(0.29896666666666666667e0_f64) * t21778 + F::cast_from(0.36514074074074074075e-1_f64) * t21802 - F::new(0.76790625e-1) * t21804 - F::cast_from(0.59793333333333333333e0_f64) * t21767 + F::new(0.17938e1) * t21774;
    (t21802, t21804, t21808)
}
