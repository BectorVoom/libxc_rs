//! GGA_X_PBEA lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbea.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_pbea_lxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t20 = M_CBRT2;
        let t21 = t20 * t20;
        let t23 = rho[ip] * rho[ip];
        let t24 = t18 * t18;
        let t26 = 1.0 / t24 / t23;
        let t29 = 1.0 + 0.86399408095363255118e-2 * sigma[ip] * t21 * t26;
        let t30 = f64::powf(t29, -0.52e0);
        let t32 = 0.1804e1 - 0.804e0 * t30;
        let t36 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t17 * t18 * t32);
        let tzk0 = 2.0 * t36;
        zk[ip] += tzk0;
        let t42 = t3 * t17;
        let t43 = t23 * rho[ip];
        let t45 = 1.0 / t18 / t43;
        let t47 = f64::powf(t29, -0.152e1);
        let t49 = t47 * sigma[ip] * t21;
        let t53 = piecewise3(t2, 0.0, -t6 * t17 / t24 * t32 / 8.0 + 0.24663433440595303582e-2 * t42 * t45 * t49);
        let tvrho0 = 2.0 * rho[ip] * t53 + 2.0 * t36;
        vrho[ip] += tvrho0;
        let t62 = piecewise3(t2, 0.0, -0.92487875402232388432e-3 * t42 / t18 / t23 * t47 * t21);
        let tvsigma0 = 2.0 * rho[ip] * t62;
        vsigma[ip] += tvsigma0;
        let t71 = t23 * t23;
        let t73 = 1.0 / t18 / t71;
        let t77 = t71 * t43;
        let t78 = 1.0 / t77;
        let t79 = t42 * t78;
        let t80 = f64::powf(t29, -0.252e1);
        let t81 = sigma[ip] * sigma[ip];
        let t83 = t80 * t81 * t20;
        let t87 = piecewise3(t2, 0.0, t6 * t17 / t24 / rho[ip] * t32 / 12.0 - 0.73990300321785910746e-2 * t42 * t73 * t49 + 0.17274545052360375959e-3 * t79 * t83);
        let tv2rho20 = 2.0 * rho[ip] * t87 + 4.0 * t53;
        v2rho2[ip] += tv2rho20;
        let t94 = t71 * t23;
        let t95 = 1.0 / t94;
        let t98 = t80 * t20 * sigma[ip];
        let t102 = piecewise3(t2, 0.0, 0.21580504260520890634e-2 * t42 * t45 * t47 * t21 - 0.64779543946351409847e-4 * t42 * t95 * t98);
        let tv2rhosigma0 = 2.0 * rho[ip] * t102 + 2.0 * t62;
        v2rhosigma[ip] += tv2rhosigma0;
        let t105 = t71 * rho[ip];
        let t111 = piecewise3(t2, 0.0, 0.24292328979881778693e-4 * t42 / t105 * t80 * t20);
        let tv2sigma20 = 2.0 * rho[ip] * t111;
        v2sigma2[ip] += tv2sigma20;
        let t119 = 1.0 / t18 / t105;
        let t123 = t71 * t71;
        let t125 = t42 / t123;
        let t128 = t123 * t23;
        let t131 = f64::powf(t29, -0.352e1);
        let t132 = 1.0 / t24 / t128 * t131;
        let t133 = t81 * sigma[ip];
        let t138 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t17 * t26 * t32 + 0.31514387174093999022e-1 * t42 * t119 * t49 - 0.17274545052360375959e-2 * t125 * t83 + 0.20059340685089964148e-4 * t42 * t132 * t133);
        let tv3rho30 = 2.0 * rho[ip] * t138 + 6.0 * t87;
        v3rho3[ip] += tv3rho30;
        let t148 = t123 * rho[ip];
        let t151 = 1.0 / t24 / t148 * t131;
        let t156 = piecewise3(t2, 0.0, -0.71935014201736302113e-2 * t42 * t73 * t47 * t21 + 0.53982953288626174872e-3 * t79 * t98 - 0.75222527569087365554e-5 * t42 * t151 * t81);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t156 + 4.0 * t102;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t165 = 1.0 / t24 / t123 * t131;
        let t170 = piecewise3(t2, 0.0, -0.12146164489940889346e-3 * t42 * t95 * t80 * t20 + 0.28208447838407762083e-5 * t42 * t165 * sigma[ip]);
        let tv3rhosigma20 = 2.0 * rho[ip] * t170 + 2.0 * t111;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t178 = piecewise3(t2, 0.0, -0.10578167939402910781e-5 * t42 / t24 / t77 * t131);
        let tv3sigma30 = 2.0 * rho[ip] * t178;
        v3sigma3[ip] += tv3sigma30;
        let t196 = t123 * t43;
        let t207 = f64::powf(t29, -0.452e1);
        let t208 = t81 * t81;
        let t214 = piecewise3(t2, 0.0, 10.0 / 27.0 * t6 * t17 / t24 / t43 * t32 - 0.16716327109736816872e0 * t42 / t18 / t94 * t49 + 0.16026939020801015473e-1 * t42 / t148 * t83 - 0.41455970749185925906e-3 * t42 / t24 / t196 * t131 * t133 + 0.16268174320405439894e-5 * t42 / t18 / t123 / t94 * t207 * t208 * t21);
        let tv4rho40 = 2.0 * rho[ip] * t214 + 8.0 * t138;
        v4rho4[ip] += tv4rho40;
        let t236 = piecewise3(t2, 0.0, 0.31171839487419064249e-1 * t42 * t119 * t47 * t21 - 0.42826476275643432065e-2 * t125 * t98 + 0.135400549624357258e-3 * t42 * t132 * t81 - 0.61005653701520399603e-6 * t42 / t18 / t123 / t105 * t207 * t133 * t21);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t236 + 6.0 * t156;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t256 = piecewise3(t2, 0.0, 0.72876986939645336076e-3 * t42 * t78 * t80 * t20 - 0.38551545379157274846e-4 * t42 * t151 * sigma[ip] + 0.22877120138070149851e-6 * t42 / t18 / t123 / t71 * t207 * t81 * t21);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t256 + 4.0 * t170;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t269 = piecewise3(t2, 0.0, 0.81099287535422315988e-5 * t42 * t165 - 0.85789200517763061941e-7 * t42 / t18 / t196 * t207 * sigma[ip] * t21);
        let tv4rhosigma30 = 2.0 * rho[ip] * t269 + 2.0 * t178;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t278 = piecewise3(t2, 0.0, 0.32170950194161148229e-7 * t42 / t18 / t128 * t207 * t21);
        let tv4sigma40 = 2.0 * rho[ip] * t278;
        v4sigma4[ip] += tv4sigma40;
    }
}
