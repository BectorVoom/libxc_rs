//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1199/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1199<F: Float>(t19949: F, t5642: F, t1731: F, t19889: F, t347: F, t1730: F, t18145: F, t18156: F, t18171: F, t19919: F, t19923: F, t19929: F, t19933: F, t19936: F, t19940: F, t19942: F, t19946: F, t5629: F, t5631: F, t5639: F, t6180: F, t6183: F) -> (F, F, F) {
    let t19950 = t19949 * t5642;
    let t19953 = t1731 * t347 * t19889;
    let t19955 = -t1730 * t19953 - t18145 * t6180 + 2.0 * t18156 * t19929 - 2.0 * t18171 * t19933 + t18171 * t19942 + 2.0 * t19919 * t5631 + 2.0 * t19923 * t5631 - t19936 * t5639 - t19940 * t5639 + 2.0 * t19946 * t5631 - t19950 * t5639 - t5629 * t6183;
    (t19950, t19953, t19955)
}
