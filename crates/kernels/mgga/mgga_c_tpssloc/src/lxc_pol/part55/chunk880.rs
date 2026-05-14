//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 880/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk880<F: Float>(t22646: F, t26184: F, t26187: F, t26191: F, t26195: F, t26198: F, t26200: F, t26204: F, t26207: F, t26212: F, t26217: F, t26219: F, t26221: F, t568: F, t254: F, t563: F) -> (F, F) {
    let t26223 = 0.38381794893125283518e-1 * t26184 - 0.16449340668482264365e-1 * t26187 - 0.16449340668482264365e-1 * t26191 - 0.16449340668482264365e-1 * t26195 + 0.82246703342411321825e-2 * t26198 + 0.19190897446562641759e-1 * t26200 - 0.82246703342411321825e-2 * t26204 - 0.82246703342411321825e-2 * t26207 + 0.82246703342411321825e-2 * t26212 + 0.16449340668482264365e-1 * t26217 - t22646 + t26219 * t568 + t26221 * t568;
    let t26224 = t563 * t254;
    (t26223, t26224)
}
