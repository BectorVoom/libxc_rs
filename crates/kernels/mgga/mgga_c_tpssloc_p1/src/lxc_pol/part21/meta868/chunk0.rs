//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3176/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3176<F: Float>(t11738: F, t15560: F, t15564: F, t15612: F, t15617: F, t18300: F, t19077: F, t3252: F, t3494: F, t3509: F, t3516: F, t3577: F, t3578: F, t44836: F, t44965: F, t45037: F, t4582: F, t4980: F, t4984: F, t5005: F, t5024: F, t52621: F, t52628: F, t52649: F, t52653: F, t52664: F, t52903: F, t53372: F, t53399: F, t6219: F) -> F {
    let t65802 = -t52621 / F::new(1728.0) - t53399 * t4984 / F::new(768.0) + t53372 * t4980 / F::new(384.0) + t52649 / F::new(3456.0) - t5005 * t15612 / F::new(1152.0) + F::new(5.0) / F::new(5184.0) * t52653 + t52628 * t15560 / F::new(216.0) - t52903 * t15564 / F::new(432.0) + t52664 / F::new(324.0) - t3577 * t3578 * t6219 * t3252 / F::new(4608.0) + t44965 * t19077 / F::new(1536.0) + t11738 * t4582 * t18300 * t3494 / F::new(3072.0) + F::new(7.0) / F::new(1536.0) * t45037 * t4582 * t18300 * t3509 - t44836 * t4582 * t18300 * t3516 / F::new(3072.0) + t5024 * t15617 / F::new(72.0);
    t65802
}
