//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1191/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1191<F: Float>(t1196: F, t12606: F, t974: F, t3548: F, t4889: F, t14736: F, t3440: F, t14740: F, t11678: F, t1174: F, t11755: F, t11787: F, t11792: F, t11794: F, t11798: F, t11802: F, t11821: F, t1227: F, t15650: F, t15656: F, t15663: F) -> F {
    let t15666 = t1196 * t12606;
    let t15667 = t974 * t15666;
    let t15671 = t4889 * t3548 / F::cast_from(162.0_f64);
    let t15672 = t3440 * t14736;
    let t15681 = t3440 * t14740;
    let t15684 = -t1227 * t15650 / F::cast_from(1152.0_f64) + t11755 / F::cast_from(648.0_f64) + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t1227 * t15656 - t11678 * t15663 / F::cast_from(1152.0_f64) - t1174 * t15667 / F::cast_from(288.0_f64) + t15671 + t1174 * t15672 / F::cast_from(108.0_f64) + F::cast_from(5.0_f64) / F::cast_from(20736.0_f64) * t11787 + t11792 / F::cast_from(10368.0_f64) + t11794 / F::cast_from(2304.0_f64) - t11798 / F::cast_from(6912.0_f64) - t11802 / F::cast_from(3456.0_f64) - t11821 / F::cast_from(6912.0_f64) + t1174 * t15681 / F::cast_from(216.0_f64);
    t15684
}
