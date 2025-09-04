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

# 앱 번들 빌드 (macOS .app 파일)
npm run tauri build -- --bundles app

if [ $? -eq 0 ]; then
    echo "=== 빌드 완료 ==="
    echo "생성된 앱: tauri-app/src-tauri/target/release/bundle/macos/알람 타이머.app"
    echo ""
    echo "앱을 실행하려면 다음 명령어를 사용하세요:"
    echo "open 'tauri-app/src-tauri/target/release/bundle/macos/알람 타이머.app'"
else
    echo "빌드 실패"
    exit 1
fi
