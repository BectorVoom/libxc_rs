//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 698/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk698<F: Float>(t5: F, t6889: F, t8621: F, t1985: F, t1998: F, t2085: F, t214: F, t590: F, t60: F, t131: F, t8308: F, t8302: F, t112: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t8622 = t6889 * t8621;
    let t8623 = t1985 * t8622;
    let t8630 = t1998 * t2085;
    let t8631 = t214 * t8630;
    let t8632 = t1985 * t8631;
    let t8705 = F::new(1.0) / t60 / t590;
    let t8706 = t8705 * t131;
    let t8707 = t8706 * t8308;
    let t8710 = piecewise3::<f64>(t8, F::new(0.0), F::new(5.0) / F::new(36.0) * t8302 * t8707);
    let t8711 = t8710 * t112;
    (t8622, t8623, t8630, t8631, t8632, t8705, t8706, t8707, t8710, t8711)
}
