//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1373/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1373<F: Float>(t1888: F, t232: F, t6646: F, t68217: F, t105574: F, t105578: F, t105582: F, t105586: F, t105596: F, t25255: F, t5612: F, t812: F, t81599: F, t87080: F, t87140: F, t87155: F, t98363: F, t98374: F, t98380: F, t98399: F, t98416: F, t98420: F, t98446: F) -> F {
    let t105601 = t1888 * t6646 * t68217 * t232;
    let t105604 = -F::new(0.24674011002723396548e-1) * t98363 - F::new(0.57572692339687925277e-1) * t98374 + F::new(0.19190897446562641759e0) * t87080 + F::new(0.57572692339687925277e-1) * t98380 - F::new(0.49348022005446793095e-1) * t105574 - F::new(0.49348022005446793095e-1) * t105578 + F::new(0.14804406601634037928e0) * t105582 + F::new(0.12337005501361698274e-1) * t98399 + F::new(0.49348022005446793095e-1) * t105586 - t81599 + F::new(0.23029076935875170111e0) * t98416 + F::new(0.49348022005446793095e-1) * t87140 - F::new(3.0) * t812 * t25255 * t5612 - F::new(0.23029076935875170111e0) * t98420 + F::new(0.49348022005446793095e-1) * t105596 - F::new(0.49348022005446793095e-1) * t98446 - F::new(0.24674011002723396548e-1) * t105601 + F::new(0.78134368175290755733e-1) * t87155;
    t105604
}
