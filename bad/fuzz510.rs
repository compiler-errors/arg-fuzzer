
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo510(_: S6, _: S1) {}
        
        fn test510() { foo510(S3, S3, S2, S7, S6, S8); }
    