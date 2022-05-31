
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16443(_: S1, _: S2, _: S5, _: S8) {}
        
        fn test16443() { foo16443(S4, S7, S4, S1, S3, S4, S6); }
    