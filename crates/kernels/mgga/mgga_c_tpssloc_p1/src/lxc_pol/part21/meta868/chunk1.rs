//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3177/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3177<F: Float>(t15643: F, t5024: F, t19201: F, t3576: F, t3577: F, t44951: F, t6191: F, t11668: F, t15569: F, t15663: F, t15704: F, t15708: F, t15750: F, t18210: F, t18231: F, t19056: F, t3494: F, t3515: F, t3580: F, t44847: F, t4582: F, t52666: F, t52674: F, t52680: F, t52682: F, t52684: F, t52766: F, t52879: F) -> F {
    let t65803 = t5024 * t15643;
    let t65815 = t19201 * t3576;
    let t65819 = t3577 * t44951 * t6191;
    let t65835 = t65803 / F::cast_from(162.0_f64) + t52666 / F::cast_from(324.0_f64) - t44847 / F::cast_from(972.0_f64) - t52674 / F::cast_from(216.0_f64) - t52680 / F::cast_from(7776.0_f64) + t52682 / F::cast_from(1152.0_f64) - t3515 * t4582 * t19056 * t3494 / F::cast_from(3072.0_f64) - t52684 / F::cast_from(864.0_f64) - t65815 * t3580 / F::cast_from(2304.0_f64) + t65819 / F::cast_from(10368.0_f64) - t52879 * t15663 / F::cast_from(576.0_f64) + t52766 * t15704 / F::cast_from(1152.0_f64) - F::cast_from(5.0_f64) / F::cast_from(648.0_f64) * t15569 * t15750 + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t3577 * t11668 * t18231 * t15708 + F::cast_from(5.0_f64) / F::cast_from(1152.0_f64) * t3577 * t11668 * t18210 * t15708;
    t65835
}
