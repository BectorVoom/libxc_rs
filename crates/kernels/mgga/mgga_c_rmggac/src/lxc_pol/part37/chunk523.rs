//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 523/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk523<F: Float>(t14980: F, t352: F, t1356: F, t14168: F, t14217: F, t14220: F, t3292: F, t504: F, t14234: F, t14241: F, t14246: F, t14256: F, t14259: F, t14303: F, t14306: F, t14312: F, t14431: F, t14432: F, t14433: F, t14440: F, t14443: F, t14447: F, t14450: F, t14454: F, t14457: F, t14460: F, t14461: F, t14462: F, t14463: F, t14464: F, t14468: F, t14471: F, t14500: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14981 = t14980 * t352;
    let t14982 = t1356 * t14981;
    let t14983 = 0.39914139006212695214e-1 * t14982;
    let t14984 = 0.58171619854173713844e-5 * t14168;
    let t14987 = 0.32526727992809621482e-5 * t14217;
    let t14988 = 0.32526727992809621482e-5 * t14220;
    let t14989 = t504 * t3292;
    let t14990 = 0.19957069503106347607e-1 * t14989;
    let t14991 = 0.72714524817717142305e-5 * t14234;
    let t14993 = 0.58171619854173713844e-5 * t14241;
    let t14994 = 0.17451485956252114153e-4 * t14246;
    let t14995 = 0.58171619854173713844e-5 * t14256;
    let t14996 = 0.58171619854173713844e-5 * t14259;
    let t15000 = t14431 - t14432 - t14433 - t14440 - t14443 + t14447 - t14450 - t14454 + t14457 + t14460 - t14461 + t14462 - t14463 - t14464 - 0.93188427318671584242e-2 * t14303 + 0.15531404553111930707e-1 * t14306 + 0.31062809106223861414e-2 * t14312 + t14468 + t14471 - t14500;
    (t14981, t14983, t14984, t14987, t14988, t14990, t14991, t14993, t14994, t14995, t14996, t15000)
}
