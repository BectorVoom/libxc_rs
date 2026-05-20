//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1646/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1646<F: Float>(t1196: F, t12606: F, t974: F, t3548: F, t4889: F, t14736: F, t3440: F, t14740: F, t11678: F, t1174: F, t11755: F, t11787: F, t11792: F, t11794: F, t11798: F, t11802: F, t11821: F, t1227: F, t15650: F, t15656: F, t15663: F) -> (F, F) {
    let t15666 = t1196 * t12606;
    let t15667 = t974 * t15666;
    let t15671 = t4889 * t3548 / F::new(162.0);
    let t15672 = t3440 * t14736;
    let t15681 = t3440 * t14740;
    let t15684 = -t1227 * t15650 / F::new(1152.0) + t11755 / F::new(648.0) + F::new(5.0) / F::new(2304.0) * t1227 * t15656 - t11678 * t15663 / F::new(1152.0) - t1174 * t15667 / F::new(288.0) + t15671 + t1174 * t15672 / F::new(108.0) + F::new(5.0) / F::new(20736.0) * t11787 + t11792 / F::new(10368.0) + t11794 / F::new(2304.0) - t11798 / F::new(6912.0) - t11802 / F::new(3456.0) - t11821 / F::new(6912.0) + t1174 * t15681 / F::new(216.0);
    (t15667, t15684)
}
