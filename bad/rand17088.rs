
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo17088(_: S1, _: S7, _: S6, _: S3, _: S5, _: S8, _: S4) {}
        
        fn test17088() { foo17088(S7, S6, S2, S3, S3, S7); }
    