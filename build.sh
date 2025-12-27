#!/bin/bash

# Tauri 앱 빌드 스크립트
# Back Health Timer Application 빌드 및 설치파일 생성

echo "=== Tauri 앱 빌드 시작 ==="

# tauri-app 디렉토리로 이동
cd tauri-app

# 의존성 설치 확인
echo "의존성 설치 확인 중..."
npm install

echo "빌드 중... (시간이 걸릴 수 있습니다)"

OS_NAME="$(uname -s)"
IS_WINDOWS=0

case "$OS_NAME" in
    MINGW*|MSYS*|CYGWIN*)
        IS_WINDOWS=1
        ;;
esac

if [ "$OS_NAME" = "Darwin" ]; then
    # 앱 번들 빌드 (macOS .app 파일)
    npm run tauri build -- --bundles app
elif [ "$IS_WINDOWS" -eq 1 ]; then
    # 앱 번들 빌드 (Windows NSIS)
    npm run tauri build -- --bundles nsis
elif [ "$OS_NAME" = "Linux" ]; then
    # 앱 번들 빌드 (Ubuntu AppImage, deb)
    npm run tauri build -- --bundles appimage,deb
else
    echo "지원하지 않는 OS입니다: $OS_NAME"
    exit 1
fi

if [ $? -eq 0 ]; then
    echo "=== 빌드 완료 ==="
    if [ "$OS_NAME" = "Darwin" ]; then
        echo "생성된 앱: tauri-app/src-tauri/target/release/bundle/macos/alarm-timer.app"
        echo ""
        echo "앱을 실행하려면 다음 명령어를 사용하세요:"
        echo "open 'tauri-app/src-tauri/target/release/bundle/macos/alarm-timer.app'"
    elif [ "$IS_WINDOWS" -eq 1 ]; then
        echo "생성된 설치 파일 위치: tauri-app/src-tauri/target/release/bundle/nsis/"
    elif [ "$OS_NAME" = "Linux" ]; then
        echo "생성된 AppImage: tauri-app/src-tauri/target/release/bundle/appimage/alarm-timer_0.1.0_amd64.AppImage"
        echo "생성된 deb: tauri-app/src-tauri/target/release/bundle/deb/alarm-timer_0.1.0_amd64.deb"
        echo ""
        echo "AppImage 실행:"
        echo "chmod +x 'tauri-app/src-tauri/target/release/bundle/appimage/alarm-timer_0.1.0_amd64.AppImage'"
        echo "'tauri-app/src-tauri/target/release/bundle/appimage/alarm-timer_0.1.0_amd64.AppImage'"
    fi
else
    echo "빌드 실패"
    exit 1
fi
