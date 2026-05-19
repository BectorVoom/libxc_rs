//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1017/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1017<F: Float>(t11605: F, t2154: F, t5059: F, t225: F, t8055: F, t2123: F, t4930: F, t1238: F, t1252: F, t14972: F, t15820: F, t1761: F, t2121: F, t2155: F, t24646: F, t24893: F, t27549: F, t27761: F, t27767: F, t27770: F, t27776: F, t27779: F, t27784: F, t3593: F, t4945: F, t5060: F, t7283: F, t7351: F, t7356: F, t8088: F) -> (F, F, F, F) {
    let t27785 = t11605 * t2154;
    let t27786 = t27785 * t5059;
    let t27792 = t8055 * t225;
    let t27794 = t4930 * t2123;
    let t27797 = -t24893 * t1761 + F::new(2.0) * t1238 * t27761 + F::cast_from(0.27415567780803773942e-2_f64) * t24646 + F::cast_from(0.82246703342411321825e-2_f64) * t2121 * t27767 - F::cast_from(0.27415567780803773942e-2_f64) * t27770 + F::new(2.0) * t4945 * t7356 + F::cast_from(0.36554090374405031923e-2_f64) * t27549 * t27776 - F::cast_from(0.27415567780803773942e-2_f64) * t7283 * t27779 + F::new(2.0) * t7351 * t5060 - F::new(6.0) * t27784 * t27786 - t15820 * t2155 - t3593 * t8088 - t14972 * t2155 - t27792 * t1252 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t27794;
    (t27785, t27786, t27792, t27797)
}
