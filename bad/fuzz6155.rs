
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6155(_: S1, _: S2, _: S5) {}
        
        fn test6155() { foo6155(S6, S4, S1, S3, S7, S2, S8); }
    