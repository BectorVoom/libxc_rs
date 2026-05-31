//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3176/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3176<F: Float>(t11738: F, t15560: F, t15564: F, t15612: F, t15617: F, t18300: F, t19077: F, t3252: F, t3494: F, t3509: F, t3516: F, t3577: F, t3578: F, t44836: F, t44965: F, t45037: F, t4582: F, t4980: F, t4984: F, t5005: F, t5024: F, t52621: F, t52628: F, t52649: F, t52653: F, t52664: F, t52903: F, t53372: F, t53399: F, t6219: F) -> F {
    let t65802 = -t52621 / F::cast_from(1728.0_f64) - t53399 * t4984 / F::cast_from(768.0_f64) + t53372 * t4980 / F::cast_from(384.0_f64) + t52649 / F::cast_from(3456.0_f64) - t5005 * t15612 / F::cast_from(1152.0_f64) + F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t52653 + t52628 * t15560 / F::cast_from(216.0_f64) - t52903 * t15564 / F::cast_from(432.0_f64) + t52664 / F::cast_from(324.0_f64) - t3577 * t3578 * t6219 * t3252 / F::cast_from(4608.0_f64) + t44965 * t19077 / F::cast_from(1536.0_f64) + t11738 * t4582 * t18300 * t3494 / F::cast_from(3072.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t45037 * t4582 * t18300 * t3509 - t44836 * t4582 * t18300 * t3516 / F::cast_from(3072.0_f64) + t5024 * t15617 / F::cast_from(72.0_f64);
    t65802
}
