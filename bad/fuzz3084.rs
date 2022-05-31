
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3084(_: S7, _: S5, _: S3, _: S4) {}
        
        fn test3084() { foo3084(S2, S5, S6, S7, S8); }
    