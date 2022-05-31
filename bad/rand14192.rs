
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo14192(_: S2, _: S4) {}
        
        fn test14192() { foo14192(S3, S2, S5); }
    