
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5243(_: S8, _: S6, _: S3, _: S2) {}
        
        fn test5243() { foo5243(S1, S2, S5, S6, S7, S8); }
    