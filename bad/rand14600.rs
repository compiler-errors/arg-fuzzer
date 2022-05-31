
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo14600(_: S2, _: S3, _: S5, _: S6, _: S7) {}
        
        fn test14600() { foo14600(S1, S2, S3, S5, S7, S8); }
    