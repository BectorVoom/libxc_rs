//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1334/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1334<F: Float>(t12451: F, t12455: F, t12460: F, t12467: F, t12483: F, t12487: F, t12492: F, t12498: F, t12503: F, t12507: F, t12512: F, t19077: F, t19090: F, t20837: F, t3107: F, t6013: F, t63250: F, t63254: F, t63258: F, t63282: F, t68361: F, t68365: F, t68373: F, t68387: F, t68391: F, t68393: F, t68394: F) -> (F,) {
    let t68399 = t68361 + t19077 * t12467 / 384.0 - t68365 + t19077 * t12498 / 768.0 + t63254 * t12503 / 256.0 - t63258 * t12507 / 256.0 - t68373 + 5.0 / 3456.0 * t6013 * t12483 + 5.0 / 6912.0 * t6013 * t12487 + 5.0 / 1152.0 * t6013 * t12492 - t19090 * t12460 / 768.0 - t19090 * t12451 / 1536.0 - 5.0 / 2592.0 * t6013 * t12512 + t68387 + t63282 * t12455 / 1536.0 - t68391 - t68393 - t68394 / 6912.0 + t63250 / 1152.0 + t20837 * t3107 / 432.0;
    (t68399,)
}
