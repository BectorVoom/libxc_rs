//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2604/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2604<F: Float>(t13969: F, t15621: F, t3506: F, t11791: F, t5005: F, t11697: F, t15477: F, t3577: F, t11677: F, t15027: F, t11680: F, t11684: F, t1174: F, t11751: F, t1177: F, t1227: F, t15740: F, t3440: F, t4582: F, t45997: F, t4889: F, t4972: F, t50873: F, t50884: F, t50959: F, t50964: F) -> F {
    let t52859 = t3506 * t13969 * t15621;
    let t52872 = t5005 * t11791;
    let t52873 = t52872 / F::new(6912.0);
    let t52875 = t3577 * t11697 * t15477;
    let t52879 = t15027 * t11677;
    let t52886 = -t1174 * t1177 * t50873 / F::new(144.0) + t52859 / F::new(768.0) + t4889 * t11751 / F::new(18.0) + t1174 * t3440 * t50884 / F::new(72.0) + t1174 * t3440 * t50959 / F::new(72.0) + t1174 * t3440 * t50964 / F::new(12.0) + t52873 - t52875 / F::new(1152.0) - t15740 * t11684 / F::new(1536.0) - t52879 * t11680 / F::new(768.0) - t1227 * t4582 * t4972 * t45997 / F::new(768.0);
    t52886
}
