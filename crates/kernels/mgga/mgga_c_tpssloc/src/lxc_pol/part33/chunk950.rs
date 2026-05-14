//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 950/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk950<F: Float>(t21762: F, t3297: F, t136: F, t1113: F, t21769: F, t21776: F, t11219: F, t21758: F, t11243: F, t21785: F, t21760: F, t21764: F, t21767: F, t21771: F, t21774: F, t21778: F, t21781: F, t21783: F, t21786: F) -> (F, F, F, F, F, F) {
    let t21788 = t3297 * t21762;
    let t21789 = t136 * t21788;
    let t21791 = t1113 * t21769;
    let t21792 = t136 * t21791;
    let t21794 = t1113 * t21776;
    let t21795 = t136 * t21794;
    let t21801 = t11219 * t21758;
    let t21802 = t136 * t21801;
    let t21804 = t11243 * t21785;
    let t21808 = 0.3071625e0 * t21781 + 0.1898925e1 * t21783 + 0.142419375e1 * t21786 - 0.16431333333333333333e0 * t21789 + 0.49293999999999999999e0 * t21792 + 0.82156666666666666667e-1 * t21795 + 0.33218518518518518518e0 * t21760 - 0.11958666666666666667e1 * t21764 + 0.17938e1 * t21771 + 0.29896666666666666667e0 * t21778 + 0.36514074074074074075e-1 * t21802 - 0.76790625e-1 * t21804 - 0.59793333333333333333e0 * t21767 + 0.17938e1 * t21774;
    (t21789, t21792, t21795, t21802, t21804, t21808)
}
