//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2992/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2992<F: Float>(t17171: F, t2970: F, t973: F, t17167: F, t10390: F, t10413: F, t14189: F, t14213: F, t17923: F, t18025: F, t2979: F, t3071: F, t43200: F, t43214: F, t43219: F, t43221: F, t43361: F, t4644: F, t48477: F, t50183: F, t50189: F, t50229: F, t5873: F, t59755: F, t59763: F, t977: F) -> F {
    let t62631 = t973 * t2970 * t17171;
    let t62640 = t973 * t2970 * t17167;
    let t62648 = -t50183 / F::new(1728.0) - t50189 / F::new(216.0) - t43361 * t3071 * t5873 * t14213 / F::new(384.0) - t10413 * t3071 * t48477 * t17923 / F::new(1152.0) + F::new(5.0) / F::new(2592.0) * t4644 * t14189 - t43200 / F::new(10368.0) - t62631 / F::new(108.0) + t973 * t977 * t59763 / F::new(48.0) + t973 * t2979 * t59755 / F::new(6.0) + t62640 / F::new(72.0) + t43214 / F::new(1944.0) + t43219 / F::new(5184.0) + t43221 / F::new(1296.0) - t10390 * t18025 / F::new(576.0) - t50229 / F::new(216.0);
    t62648
}
