//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 784/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk784<F: Float>(t14404: F, t15071: F, t15430: F, t15431: F, t15432: F, t15433: F, t15434: F, t15435: F, t15438: F, t15441: F, t15444: F, t15446: F, t70657: F, t70658: F, t70659: F, t14406: F, t14407: F, t14408: F, t14409: F, t14410: F, t14411: F, t14423: F, t14426: F, t15452: F, t15453: F, t15454: F, t15455: F, t70661: F, t70667: F, t70668: F) -> (F, F) {
    let t76587 = t70657 - t15430 + t15431 + t15432 + t15071 - t15433 - t15434 + t15435 + t15438 + t70658 - t15441 + t15444 + t15446 - t70659 - t14404;
    let t76589 = t70661 + t14406 + t14407 + t15452 - t14408 + t14409 - t14410 - t14411 - t15453 + t15454 + t15455 - t70667 + t70668 + t14423 - t14426;
    (t76587, t76589)
}
