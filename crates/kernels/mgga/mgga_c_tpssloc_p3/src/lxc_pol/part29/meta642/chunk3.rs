//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2116/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2116<F: Float>(t13306: F, t23146: F, t13231: F, t25084: F, t81789: F, t81795: F, t81797: F, t81799: F, t81810: F, t81825: F, t81836: F, t81850: F, t81853: F, t87263: F, t87268: F, t87271: F, t87273: F, t87274: F, t87276: F, t87278: F) -> F {
    let t87280 = t23146 * t13306;
    let t87284 = t25084 * t13231;
    let t87286 = t87263 - F::cast_from(0.63250651214153279005e-2_f64) * t81789 - F::cast_from(0.14130464632949136799e-2_f64) * t81795 - F::cast_from(0.28260929265898273598e-2_f64) * t81797 + F::new(7.0) / F::new(144.0) * t81799 - t87268 + F::new(7.0) / F::new(2304.0) * t81810 - t87271 + t87273 + t87274 / F::new(768.0) + t87276 / F::new(384.0) + t87278 / F::new(384.0) + t87280 / F::new(384.0) + F::new(7.0) / F::new(1152.0) * t81825 - F::cast_from(0.16956557559538964159e-1_f64) * t81836 - t81850 - t81853 - t87284 / F::new(96.0);
    t87286
}
