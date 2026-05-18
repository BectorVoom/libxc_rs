//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1154/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1154<F: Float>(t1227: F, t18975: F, t4997: F, t5019: F, t4993: F, t5005: F, t1202: F, t6164: F, t5024: F, t11692: F, t11792: F, t11821: F, t15671: F, t15691: F, t15699: F, t15740: F, t18955: F, t18959: F, t18965: F, t18969: F, t18972: F, t3577: F, t488: F, t4950: F) -> F {
    let t18976 = t1227 * t18975;
    let t18978 = t5019 * t4997;
    let t18980 = t5005 * t4993;
    let t18982 = t1202 * t6164;
    let t18987 = t5024 * t4993;
    let t18989 = -F::new(5.0) / F::new(5184.0) * t1227 * t18955 - t1227 * t18959 / F::new(2304.0) - t15740 * t4950 / F::new(2304.0) + t11692 * t18965 / F::new(4608.0) - t3577 * t18969 / F::new(4608.0) + t15671 + t18972 / F::new(2304.0) + F::new(5.0) / F::new(20736.0) * t18976 - t18978 / F::new(432.0) - t18980 / F::new(3456.0) + F::new(19.0) / F::new(1728.0) * t18982 * t488 + t11792 / F::new(20736.0) - t11821 / F::new(13824.0) + t18987 / F::new(648.0) - t15691 + t15699;
    t18989
}
