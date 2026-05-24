//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1157/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1157<F: Float>(t12920: F, t2206: F, t4438: F, t10039: F, t10028: F, t10038: F, t10042: F, t12907: F, t12909: F, t12911: F, t12913: F, t12915: F, t12918: F, t12919: F, t7979: F, t7988: F, t7992: F) -> (F, F, F, F) {
    let t12921 = F::cast_from(0.17315859105681463759e2_f64) * t12920;
    let t12922 = t4438 * t2206;
    let t12923 = F::cast_from(0.5848223622634646207e0_f64) * t12922;
    let t12924 = F::new(4.0) * t10039;
    let t12925 = -t10028 - t12907 + t7979 + t12909 - t12911 + t12913 - t12915 + t12918 + t12919 - t12921 - t12923 - t10038 + t12924 - t10042 + t7988 + t7992;
    (t12921, t12923, t12924, t12925)
}
