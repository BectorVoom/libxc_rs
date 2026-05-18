//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1341/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1341<F: Float>(t1799: F, t2105: F, t5815: F, t645: F, t5935: F, t9895: F, t1692: F, t1989: F, t5849: F, t18728: F, t63785: F, t17921: F, t17934: F, t1812: F, t18803: F, t18807: F, t19825: F, t1991: F, t20510: F, t2439: F, t3552: F, t5539: F, t5853: F, t6120: F, t6354: F, t63766: F, t63823: F, t63873: F, t63877: F, t63885: F, t64273: F) -> (F, F, F, F, F, F) {
    let t66195 = t2105 * t1799;
    let t66199 = t645 * t5815;
    let t66217 = t5935 * t9895;
    let t66235 = F::new(2.0) * t1692 * t5849 * t1989;
    let t66262 = F::new(6.0) * t18728 * t63785;
    let t66266 = F::new(3.0) * t2439 * t6354 * t17934 - F::new(3.0) * t18728 * t63766 + t66235 + F::new(3.0) * t2439 * t20510 * t5539 + F::new(6.0) * t18728 * t63885 - t1692 * t18807 * t19825 + F::new(3.0) / F::new(2.0) * t2439 * t1812 * t63873 + t1692 * t6354 * t1991 / F::new(2.0) + F::new(3.0) * t2439 * t1812 * t63877 - t1692 * t5853 * t64273 / F::new(2.0) + F::new(3.0) * t3552 * t1812 * t63823 + F::new(3.0) * t3552 * t6354 * t17921 + t66262 + F::new(3.0) / F::new(2.0) * t2439 * t18803 * t6120;
    (t66195, t66199, t66217, t66235, t66262, t66266)
}
