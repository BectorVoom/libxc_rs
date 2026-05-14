//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 983/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk983<F: Float>(t21760: F, t21764: F, t21767: F, t21771: F, t21774: F, t21778: F, t21781: F, t21783: F, t21786: F, t21789: F, t21792: F, t21795: F, t21802: F, t21804: F, t21922: F, t1156: F) -> (F, F) {
    let t21937 = 0.16504875e0 * t21781 + 0.258925e1 * t21783 + 0.19419375e1 * t21786 - 0.16557e0 * t21789 + 0.49671e0 * t21792 + 0.82785e-1 * t21795 + 0.33547222222222222222e0 * t21760 - 0.12077e1 * t21764 + 0.181155e1 * t21771 + 0.301925e0 * t21778 + 0.36793333333333333333e-1 * t21802 - 0.412621875e-1 * t21804 - 0.60384999999999999999e0 * t21767 + 0.181155e1 * t21774;
    let t21938 = t21922 + t21937;
    let t21939 = t21938 * t1156;
    (t21938, t21939)
}
