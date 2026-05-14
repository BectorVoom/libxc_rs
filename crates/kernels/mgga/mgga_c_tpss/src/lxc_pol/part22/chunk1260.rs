//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1260/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1260<F: Float>(t20217: F, t508: F, t10456: F, t1163: F, t1322: F, t1600: F, t1760: F, t1844: F, t18551: F, t18627: F, t18680: F, t18896: F, t19577: F, t20226: F, t20288: F, t20361: F, t20386: F, t2054: F, t2056: F, t20640: F, t3166: F, t3245: F, t3491: F, t4341: F, t485: F, t5706: F, t5709: F, t5799: F, t5895: F, t5910: F, t624: F, t6245: F, t626: F, t6309: F, t63101: F, t6324: F, t6399: F, t67519: F, t7798: F) -> (F,) {
    let t67782 = t508 * t20217;
    let t67792 = -2.0 * t626 * t1600 * t18627 - 2.0 * t7798 * t6324 - 4.0 * t10456 * t6324 - 4.0 * t2056 * t20386 - t1322 * t18896 - t18680 * t1600 - 2.0 * t5799 * t4341 - t67519 * t485 - 2.0 * t20288 * t1163 - t2054 * t6399 - 2.0 * t624 * t20640 + 3.0 * t1760 * t20226 * t18551 + 6.0 * t19577 * t5910 + 3.0 * t1760 * t63101 * t6245 - 2.0 * t3491 * t5895 - t6309 * t3166 + 6.0 * t1760 * t67782 * t5709 + 6.0 * t1760 * t3245 * t1844 * t6245 - 2.0 * t5706 * t20361;
    (t67792,)
}
