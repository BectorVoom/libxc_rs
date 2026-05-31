//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2621/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2621<F: Float>(t11148: F, t11665: F, t11674: F, t11678: F, t11724: F, t11766: F, t11855: F, t1216: F, t14706: F, t15470: F, t15661: F, t15663: F, t15740: F, t1735: F, t18946: F, t3577: F, t3578: F, t3580: F, t45128: F, t45162: F, t45211: F, t4889: F, t5019: F, t53322: F, t53336: F) -> F {
    let t53345 = -F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t3577 * t45128 * t1735 * t11148 + F::cast_from(7.0_f64) / F::cast_from(243.0_f64) * t4889 * t11766 + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t45211 - t53322 * t3580 / F::cast_from(768.0_f64) - t15740 * t11674 / F::cast_from(1536.0_f64) - t11665 * t15470 / F::cast_from(768.0_f64) - t3577 * t3578 * t14706 * t1216 / F::cast_from(1536.0_f64) - t5019 * t11855 / F::cast_from(576.0_f64) - t53336 * t11724 / F::cast_from(96.0_f64) - t45162 * t15663 / F::cast_from(384.0_f64) - t11678 * t3578 * t18946 * t15661 / F::cast_from(384.0_f64);
    t53345
}
