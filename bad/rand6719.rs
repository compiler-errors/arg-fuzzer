
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6719(_: S4, _: S6, _: S7) {}
        
        fn test6719() { foo6719(S2, S3, S5, S6, S8); }
    