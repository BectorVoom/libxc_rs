//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1165/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1165<F: Float>(t1902: F, t9971: F, t226: F, t23008: F, t23016: F, t235: F, t2617: F, t6657: F, t812: F, t81689: F, t81691: F, t81695: F, t81697: F, t81702: F, t81704: F, t81709: F, t81713: F, t81717: F, t81718: F, t81976: F, t81980: F, t81987: F, t81989: F, t829: F, t9661: F, t9976: F, t9981: F) -> (F,) {
    let t81991 = t9971 * t1902;
    let t82000 = -t81689 + 0.12337005501361698274e-1 * t81691 + 0.14804406601634037928e0 * t81695 + 0.57572692339687925277e-1 * t81697 - 0.24674011002723396548e-1 * t81702 + 0.57572692339687925277e-1 * t81704 - 0.24674011002723396548e-1 * t81709 + 0.49348022005446793095e-1 * t81713 + t81717 - 3.0 * t812 * t81718 * t829 - 3.0 * t2617 * t23016 + t226 * t235 * t81976 - 0.34543615403812755166e0 * t81980 - 0.19739208802178717238e0 * t81987 + 0.11514538467937585055e0 * t81989 - 6.0 * t812 * t81991 * t9976 + 6.0 * t812 * t23008 * t9981 - t812 * t6657 * t9661;
    (t82000,)
}
