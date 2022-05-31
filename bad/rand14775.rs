
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo14775(_: S2, _: S3, _: S4, _: S6, _: S7) {}
        
        fn test14775() { foo14775(S3, S3, S2, S3, S7, S4, S3); }
    