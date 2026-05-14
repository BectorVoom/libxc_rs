//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1256/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1256<F: Float>(t25: F, t265: F, t394: F, t121907: F, t121949: F, t121283: F, t121798: F, t121833: F, t121865: F, t1409: F, t31478: F, t33513: F, t3966: F, t40: F, t607: F, t8580: F, t100688: F, t101840: F, t119746: F, t119780: F, t121264: F, t1877: F, t24191: F, t2522: F, t25901: F, t25930: F, t26744: F, t26756: F, t30974: F, t31434: F, t31441: F, t31448: F, t31502: F, t33483: F, t33537: F, t7114: F, t8566: F, t89849: F, t89992: F, t92271: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t121950 = t121907 + t121949;
    let t121951 = piecewise3(t395, 0.0, t121950);
    let t121958 = piecewise3(t115, t121283 + t121798 + t121833 + t121865, t121951 * t40 / 2.0 + t31478 * t1409 / 2.0 + t33513 * t607 / 2.0 + t8580 * t3966 / 2.0);
    let t121982 = t26756 * t89849 * t33483 + 3.0 / 2.0 * t2522 * t8566 * t25901 + t101840 * t31502 - t1877 * t7114 * t119746 / 2.0 - 3.0 / 2.0 * t24191 * t89992 * t31441 - t1877 * t26744 * t30974 / 2.0 + t92271 * t33537 + t121264 + t26756 * t100688 * t31448 - t1877 * t31434 * t25930 / 2.0 - 3.0 / 2.0 * t24191 * t119780;
    (t121950, t121958, t121982)
}
