//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 911/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk911<F: Float>(t14182: F, t14654: F, t14655: F, t14656: F, t14659: F, t14660: F, t14661: F, t14662: F, t14663: F, t15515: F, t15518: F, t15520: F, t15522: F, t15525: F, t76627: F, t76645: F, t76685: F, t76709: F, t76754: F, t76776: F, t76806: F, t76824: F, t76845: F, t76862: F, t76881: F, t76900: F, t76934: F, t76953: F, t76971: F, t76989: F, t77013: F, t77038: F, t77067: F, t77101: F, t77120: F, t77139: F, t77163: F, t77179: F, t77200: F, t77226: F, t77245: F, t77261: F, t77298: F, t77333: F, t77368: F, t77407: F, t77433: F, t77448: F, t77475: F, t77492: F, t77513: F, t77538: F, t77568: F, t77599: F, t77632: F, t77648: F, t77661: F, t77682: F, t77706: F, t77727: F, t77776: F, t77798: F, t77829: F, t78282: F, t78305: F, t78317: F, t78348: F, t78373: F, t78392: F, t78413: F, t78437: F, t78470: F, t78485: F, t78505: F, t78532: F, t78562: F, t78589: F, t78614: F, t8: F) -> (F,) {
    let t78622 = -t15515 + t15518 - t15520 + t8 * (t78614 + t78589 + t78562 + t78532 + t78505 + t78485 + t78470 + t78437 + t78413 + t78392 + t78373 + t78348 + t78317 + t78305 + t78282 + t77829 + t77798 + t77776 + t77727 + t77706 + t77682 + t77661 + t77648 + t77632 + t77599 + t77568 + t77538 + t77513 + t77492 + t77475 + t77448 + t77433 + t77407 + t77368 + t77333 + t77298 + t77261 + t77245 + t77226 + t77200 + t77179 + t77163 + t77139 + t77120 + t77101 + t77067 + t77038 + t77013 + t76989 + t76971 + t76953 + t76934 + t76900 + t76881 + t76862 + t76845 + t76824 + t76806 + t76776 + t76754 + t76709 + t76685 + t76645 + t76627) - t14654 - t15522 - t15525 - t14655 - t14656 + t14182 + t14659 - t14660 + t14661 + t14662 - t14663;
    (t78622,)
}
