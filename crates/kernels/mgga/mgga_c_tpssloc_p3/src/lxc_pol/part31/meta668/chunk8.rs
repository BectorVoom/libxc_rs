//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1973/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1973<F: Float>(t84921: F, t84932: F, t87437: F, t87438: F, t87440: F, t87445: F, t92697: F, t92705: F, t92710: F, t92713: F, t98847: F, t98849: F, t98851: F, t98853: F, t98858: F, t98862: F, t98868: F, t98871: F) -> F {
    let t101496 = t98847 / F::new(192.0) - F::new(5.0) / F::new(192.0) * t98849 + t98851 / F::new(96.0) - t98853 / F::new(384.0) - t84921 + t87437 - F::cast_from(0.40372756094140390853e-3_f64) * t98858 + F::cast_from(0.24223653656484234512e-2_f64) * t98862 - t87438 - t87440 + t92697 + F::cast_from(0.40372756094140390853e-3_f64) * t87445 - t84932 - t92705 + t98868 / F::new(8.0) + F::cast_from(0.16956557559538964158e-1_f64) * t98871 - t92710 + t92713;
    t101496
}
