//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1259/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1259<F: Float>(t10461: F, t12664: F, t13133: F, t13235: F, t1339: F, t13554: F, t1663: F, t1796: F, t1800: F, t1846: F, t18613: F, t18919: F, t19308: F, t19577: F, t20219: F, t20386: F, t20396: F, t3396: F, t3499: F, t42336: F, t485: F, t5706: F, t5801: F, t5809: F, t5816: F, t5937: F, t6103: F, t62230: F, t6318: F, t6324: F, t6409: F, t65067: F, t65941: F, t67316: F) -> (F,) {
    let t67751 = -2.0 * t67316 * t485 - 2.0 * t13235 * t6324 - 4.0 * t3499 * t20386 - 2.0 * t62230 * t1339 + t6409 * t3396 + t18919 * t1663 - 2.0 * t6103 * t18613 - 4.0 * t13133 * t5816 - 2.0 * t42336 * t1800 - 4.0 * t13554 * t5809 - 2.0 * t13235 * t6318 - 4.0 * t3499 * t20396 - 2.0 * t65067 * t1800 - 4.0 * t19308 * t5809 + 2.0 * t5706 * t20219 - t1796 * t12664 + 2.0 * t19577 * t5937 + t65941 * t1846 - 4.0 * t5801 * t10461;
    (t67751,)
}
