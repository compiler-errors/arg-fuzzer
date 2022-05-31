
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo14763(_: S1, _: S2, _: S4) {}
        
        fn test14763() { foo14763(S3, S7, S1, S6, S5, S5); }
    