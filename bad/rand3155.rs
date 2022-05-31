
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3155(_: S1, _: S2, _: S3) {}
        
        fn test3155() { foo3155(S6, S4, S1, S4, S3); }
    