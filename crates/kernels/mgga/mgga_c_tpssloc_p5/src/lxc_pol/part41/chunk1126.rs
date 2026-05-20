//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1126/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1126<F: Float>(t17933: F, t17958: F, t360: F, t1021: F, t248: F, t1020: F, t10413: F, t10891: F, t10949: F, t14077: F, t14080: F, t14136: F, t14139: F, t14207: F, t1618: F, t1622: F, t17907: F, t17920: F, t17925: F, t3048: F, t3070: F, t4641: F, t4652: F, t5857: F, t5875: F, t5880: F, t5900: F) -> (F, F) {
    let t17959 = t17933 + t17958;
    let t17960 = t17959 * t360;
    let t17962 = t248 * t1021 * t17960;
    let t17967 = -t14080 * t1622 / F::new(432.0) + t3048 * t5900 / F::new(432.0) - t17907 / F::new(3456.0) + t10891 * t5880 / F::new(576.0) - t3048 * t5857 / F::new(864.0) + t14207 * t1618 / F::new(1536.0) + t4641 * t4652 / F::new(1536.0) - t14077 * t1618 / F::new(288.0) + F::new(5.0) / F::new(6912.0) * t3070 * t17920 - t10413 * t17925 / F::new(2304.0) - t14136 + t14139 + t1020 * t17962 / F::new(3072.0) + t10949 * t5875 / F::new(1536.0);
    (t17959, t17967)
}
