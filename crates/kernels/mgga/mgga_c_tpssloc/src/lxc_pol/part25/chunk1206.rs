//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1206/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1206<F: Float>(t81849: F, t81852: F, t81855: F, t81857: F, t81859: F, t81861: F, t81863: F, t81866: F, t81869: F, t81874: F, t81877: F, t81880: F, t81883: F, t81887: F, t81889: F, t81891: F, t81893: F, t81895: F, t81899: F, t81903: F) -> F {
    let t84896 = F::new(0.2034786907144675699e0) * t81849;
    let t84897 = F::new(455.0) / F::new(648.0) * t81852;
    let t84916 = -t84896 - t84897 - F::new(0.24223653656484234512e-2) * t81855 - F::new(35.0) / F::new(96.0) * t81857 + F::new(0.84782787797694820791e-2) * t81859 - F::new(5.0) / F::new(64.0) * t81861 + t81863 / F::new(64.0) + t81866 / F::new(32.0) - F::new(0.40372756094140390853e-3) * t81869 + F::new(0.20186378047070195427e-3) * t81874 + F::new(0.10093189023535097713e-3) * t81877 + t81880 / F::new(768.0) - F::new(0.31625325607076639502e-2) * t81883 - F::new(7.0) / F::new(192.0) * t81887 + F::new(7.0) / F::new(384.0) * t81889 + F::new(5.0) / F::new(64.0) * t81891 - t81893 / F::new(256.0) - t81895 / F::new(768.0) + F::new(0.12111826828242117256e-2) * t81899 + F::new(0.60559134141210586279e-3) * t81903;
    t84916
}
