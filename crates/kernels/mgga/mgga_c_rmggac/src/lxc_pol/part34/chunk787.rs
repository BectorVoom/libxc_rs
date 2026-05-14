//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 787/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk787<F: Float>(t14600: F, t14601: F, t14602: F, t14603: F, t14607: F, t14608: F, t14609: F, t15466: F, t15469: F, t15472: F, t15473: F, t15474: F, t15475: F, t15476: F, t70705: F, t14612: F, t14616: F, t14620: F, t14622: F, t14625: F, t14849: F, t15480: F, t15481: F, t15482: F, t15484: F, t15485: F, t15486: F, t15487: F, t15491: F, t70707: F, t70708: F) -> (F, F) {
    let t76596 = -t14600 + t14601 + t14602 + t14603 + t14607 + t14608 - t14609 - t70705 + t15466 - t15469 - t15472 + t15473 + t15474 + t15475 - t15476;
    let t76597 = -t14612 - t15480 + t70707 - t70708 - t14849 - t14616 - t14620 - t14622 - t15481 - t15482 + t15484 + t15485 + t15486 - t15487 + t15491 + t14625;
    (t76596, t76597)
}
